FROM nginx:alpine

# Copy the nginx configuration
COPY deploy/nginx.conf /etc/nginx/conf.d/default.conf

# Copy the built application from the dist directory
COPY dist /usr/share/nginx/html

# Expose port 80
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
