# Image Component Usage

The `<Image />` component allows you to embed clickable images in markdown that open as modal overlays.

## Basic Usage

```markdown
<Image path="/static/images/example.png" />
```

This renders a thumbnail image (300px width by default) that opens in a modal when clicked.

## With Custom Properties

```markdown
<Image
  path="/static/images/example.png"
  alt="Example screenshot"
  thumbnail_width="200"
  class="my-custom-class"
/>
```

## Supported Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `path` | String | Required | Path to the image file |
| `alt` | String | "" | Alt text for accessibility |
| `thumbnail_width` | Number | 300 | Width of the thumbnail in pixels |
| `class` | String | "" | Additional CSS classes |

## Examples in Context

Here's how you can use it in blog posts:

Take a look at this architecture diagram:

<Image path="/static/images/application_model_light.png" alt="Application Architecture" />

The image will appear as a clickable thumbnail that opens full-size when clicked.

For smaller thumbnails, adjust the width:

<Image path="/static/images/breadboard.png" alt="Breadboard setup" thumbnail_width="200" />

## Features

- **Responsive Modal**: Opens full-size images in a centered modal overlay
- **Keyboard Friendly**: Click anywhere outside the image to close
- **Hover Effects**: Thumbnails have subtle hover animations
- **Accessibility**: Proper alt text support for screen readers
- **Customizable**: Control thumbnail size and styling with attributes
