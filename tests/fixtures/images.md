# Images Test

This fixture validates standard markdown image syntax rendering.

---

## Missing Image (Error Handling Test)

![This image does not exist](nonexistent-image.png)

This tests graceful failure when an image file is missing.

---

## Image with Descriptive Alt Text

![A detailed description of what this image should contain for accessibility purposes](logo.png)

---

## Multiple Images

![First Image](logo.png)

Some text between images.

![Second Image](logo.png)

---

## HTML-Enhanced Images (Width Control)

<p><img src="logo.png" width="100"></p>

Small version (width="100")

<p><img src="logo.png" width="200"></p>

Medium version (width="200")

<p><img src="logo.png" width="400"></p>

Large version (width="400")

---

## HTML-Enhanced Images (Alignment)

<p align="left"><img src="logo.png" width="150"></p>

Left-aligned image (align="left")

<p align="center"><img src="logo.png" width="150"></p>

Center-aligned image (align="center")

<p align="right"><img src="logo.png" width="150"></p>

Right-aligned image (align="right")
