#!/usr/bin/env python3
from PIL import Image, ImageDraw, ImageFont

# Create image
width, height = 1200, 400
img = Image.new('RGB', (width, height), color='#0f172a')
draw = ImageDraw.Draw(img)

# Colors
success_color = (16, 185, 129)
error_color = (239, 68, 68)
neutral_color = (79, 70, 229)
text_light = (255, 255, 255)
text_success = (134, 239, 172)
text_error = (252, 165, 165)
gray = (148, 163, 184)

# Draw operation box (left)
draw.rounded_rectangle([(50, 150), (250, 250)], radius=10, fill=neutral_color)

# Try to load fonts
try:
    font_large = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 20)
    font_med = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 16)
    font_small = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 14)
    font_tiny = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 12)
except Exception as e:
    print(f"Could not load fonts: {e}")
    font_large = font_med = font_small = font_tiny = ImageFont.load_default()

# Operation box text
draw.text((150, 185), "Operation", fill=text_light, anchor="mm", font=font_large)
draw.text((150, 215), "execute()", fill=(224, 231, 255), anchor="mm", font=font_med)

# Draw branching line
draw.line([(250, 200), (400, 200)], fill=gray, width=3)

# Fork point circle
draw.ellipse([(385, 185), (415, 215)], fill=(100, 116, 139))

# Success branch (top path)
draw.line([(400, 200), (450, 120), (750, 120)], fill=success_color, width=4)
# Arrow head
draw.polygon([(750, 120), (735, 115), (735, 125)], fill=success_color)

# Success Result box
draw.rounded_rectangle([(770, 70), (1150, 170)], radius=10, outline=success_color, width=2)
draw.text((960, 100), "Result.Ok<T>", fill=success_color, anchor="mm", font=font_large)
draw.text((960, 130), "Success Value", fill=text_success, anchor="mm", font=font_small)
draw.text((960, 155), "{ value: T }", fill=text_success, anchor="mm", font=font_tiny)

# Error branch (bottom path)
draw.line([(400, 200), (450, 280), (750, 280)], fill=error_color, width=4)
# Arrow head
draw.polygon([(750, 280), (735, 275), (735, 285)], fill=error_color)

# Error Result box
draw.rounded_rectangle([(770, 230), (1150, 330)], radius=10, outline=error_color, width=2)
draw.text((960, 260), "Result.Err<E>", fill=error_color, anchor="mm", font=font_large)
draw.text((960, 290), "Error Information", fill=text_error, anchor="mm", font=font_small)
draw.text((960, 315), "{ error: E }", fill=text_error, anchor="mm", font=font_tiny)

# Branch labels
draw.text((580, 105), "✓ Success Path", fill=text_success, anchor="mm", font=font_small)
draw.text((580, 265), "✗ Failure Path", fill=text_error, anchor="mm", font=font_small)

# Decorative circles on success path
for x, r in [(500, 5), (550, 4), (600, 6), (650, 4), (700, 5)]:
    draw.ellipse([(x-r, 120-r), (x+r, 120+r)], fill=success_color)

# Decorative circles on error path
for x, r in [(500, 5), (550, 4), (600, 6), (650, 4), (700, 5)]:
    draw.ellipse([(x-r, 280-r), (x+r, 280+r)], fill=error_color)

# Save as JPEG
output_path = 'content/images/result_type_top.jpg'
img.save(output_path, 'JPEG', quality=95)
print(f"✓ Image created successfully at: {output_path}")
