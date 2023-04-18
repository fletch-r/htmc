# HTMC

This tool is for merging HTML files by replacing elements in the `index.html` file that contain a path attribute with the contents of another HTML file using the path that is given in the path attribute.

## Usage
To use the tool, simply run the following command in your terminal:

```console
$ cargo run
```

This will merge the contents of `merge.html` into `index.html`, replacing any element in `index.html` that has a path attribute with the contents of the corresponding element in `merge.html`.

This will replace any elements in the `index.html` that contain a `path=""` attribute with the contents of the file given in the path attribute.

## Example

Suppose you have an index.html file that looks like this:

```html
<html>
  <head>
    <title>My Website</title>
  </head>
  <body>
    <div path="./header.html"></div>
    <div path="../content/content.html"></div>
    <div path="../footer/footer.html"></div>
  </body>
</html>
```

And you have the following `header.html`, `content.html`, and `footer.html` files:

```html
<!-- header.html -->
<header>
  <h1>Welcome to My Website!</h1>
</header>

<!-- content.html -->
<main>
  <p>This is the main content of my website.</p>
</main>

<!-- footer.html -->
<footer>
  <p>&copy; 2023 My Company, Inc.</p>
</footer>
```

This will replace the div elements in `index.html` with the contents of the corresponding HTML files, resulting in a merged `index.html` file that looks like this:

```html
<html>
  <head>
    <title>My Website</title>
  </head>
  <body>
    <header>
      <h1>Welcome to My Website!</h1>
    </header>
    <main>
      <p>This is the main content of my website.</p>
    </main>
    <footer>
      <p>&copy; 2023 My Company, Inc.</p>
    </footer>
  </body>
</html>
```

<!-- ## Installation

To install the HTML merger tool, you'll need to have Node.js installed on your computer. You can download it from the official website.

Once you have Node.js installed, you can install the tool globally using npm:

```
npm install -g html-merge-tool
```

This will make the `html-merge` command available in your terminal. -->

## Contributing

If you'd like to contribute to the HTML merger tool, please fork this repository and submit a pull request with your changes.

## license

The HTML merger tool is licensed under the *MIT License*.