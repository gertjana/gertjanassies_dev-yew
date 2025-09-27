---
title: How to use AWS services from EKS with fine-grained access control
author: Gertjan Assies
date: "2022-11-02"
category: code
tags: aws, eks, oidc, python, boto3, featured
image: "/static/images/eks_fine_grained_access.jpg"
summary: bind service accounts to IAM roles to allow fine grained access control from k8s resources.
published: true

---

At my company, where I am part of the SRE team, we are using [EKS](https://aws.amazon.com/eks/) (Amazon's managed Kubernetes service) to run our applications and services.  
We also use it to deploy those applications and services and increasingly serverless stacks with cloud formation or terraform.

But how do we keep our systems secure and stable while allowing the dev teams to build, deploy, monitor and debug their applications as they see fit?

Kubernetes has the concept of service accounts. these are k8s resources that allow other k8s resources like pods for instance to interact with k8s and external services.

EKS allows you to bind those service accounts to IAM Roles allowing fine-grained control on what AWS Services a k8s resource can interact with.  
This is possible as EKS can authenticate by using an [OIDC Provider](https://docs.aws.amazon.com/eks/latest/userguide/enable-iam-roles-for-service-accounts.html) (follow the link to see how that connects Service accounts and IAM Roles.)

For us, this means we can give development teams the possibility to deploy their k8s deployments, cloud formation stacks and big data jobs through CI/CD pipelines in their namespaces without having to give them extensive access to the AWS Console.

How that all works in detail is for another blog entry, what I want to show here is that if you want to interact with AWS services from within your application or job, you need to be able to assume that role that is bound to that service account.

The example I'm showing uses Python and the Boto3 library, but this should be possible in any language that has an AWS client library or SDK that support STS (Secure Token Service)

```python
import os
import boto3

role_arn = os.getenv("AWS_ROLE_ARN")

web_identity_token = None
with open(os.getenv("AWS_WEB_IDENTITY_TOKEN_FILE"), 'r') as f:
  web_identity_token = f.read()

sts_client = boto3.client("sts")
sts_role_resp = sts_client.assume_role_with_web_identity(
                  RoleArn=role_arn,
                  RoleSessionName="My Session",
                  WebIdentityToken=web_identity_token
                )

sts_role_creds = {
  "AccessKeyId": sts_role_resp['Credentials']['AccessKeyId'],
  "SecretAccessKey": sts_role_resp['Credentials']['SecretAccessKey'],
  "SessionToken": sts_role_resp['Credentials']['SessionToken']
}

# code here that does whatever you need doing, that the role you assume allows you to
# in this case list all s3 bucket names
s3_resource=boto3.resource('s3',
    aws_access_key_id=sts_role_creds['AccessKeyId'],
    aws_secret_access_key=sts_role_creds['SecretAccessKey'],
    aws_session_token=sts_role_creds['SessionToken'],
)

for bucket in s3_resource.buckets.all():
    print(bucket.name)
```

As you can see once the service account role bindings are set up the following environment variables are available:

| ENV var | description |
| -- | -- |
| AWS_ROLE_ARN | The Role to assume |
| AWS_WEB_IDENTITY_TOKEN_FILE | the file location containing a token to authenticate against |

Hopefully, I've given you some ideas on how to run AWS workloads in EKS.

Thanks for reading.
