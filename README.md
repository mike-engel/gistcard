# gistcard

> Better gist previews on twitter and other open graph applications

`gistcard` is a better way to share gists (until github does this themselves 😉). It shows a snippet of the gist as a preview using [carbon](https://carbon.now.sh) and includes a link to the gist.

## Usage

The original inspiration for this was from a [tweet](https://twitter.com/noopkat/status/1138552168372289537) from [Suz Hinton](https://twitter.com/noopkat). It's a wonderful idea, and I fully expect this tool to only last for a little while until GitHub does it officially.

`gistcard` is meant to only be used on the web, so it only provides a web facing API. There are three parts to it:

1. **An image creator**: This creates the image and exposes itself under the url `/img/:gistId.png`. This uses the Carbon API to crate and serve the image as a PNG.
2. **A forwarder**: This provides a barebones HTML page that uses the existing `meta` tags from the official gist page, and adds the proper image source for the code preview. Upon visiting, it immediately redirects to the gist page.
3. **A UI**: This provides a simple landing page to demonstrate how to use it and to generate a URL to paste, if needed.

For ease of use, the gist for the page can be the full url of the gist (with or without the username), or just the ID. All three of the following are valid URLs!

- [https://gistcard.now.sh/https://gist.github.com/mike-engel/a1c11cd6d0edcdab6fd912619b33d972](https://gistcard.now.sh/https://gist.github.com/mike-engel/a1c11cd6d0edcdab6fd912619b33d972)
- [https://gistcard.now.sh/https://gist.github.com/a1c11cd6d0edcdab6fd912619b33d972](https://gistcard.now.sh/https://gist.github.com/a1c11cd6d0edcdab6fd912619b33d972)
- [https://gistcard.now.sh/a1c11cd6d0edcdab6fd912619b33d972](https://gistcard.now.sh/a1c11cd6d0edcdab6fd912619b33d972)

The image URL will always use the gist's ID, like so: [https://gistcard.now.sh/img/a1c11cd6d0edcdab6fd912619b33d972.png](https://gistcard.now.sh/img/a1c11cd6d0edcdab6fd912619b33d972.png).

## Contributing

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

I would love to see issues and pull requests to make this a better tool that works for people other than myself!

This project only works with rust's 2018 edition. Thus, you must have version 1.31 or later. Once you have rust installed, you can then run `cargo build` to see it in action. This will download and compile all the dependencies in development mode.

In theory, you can use now's [`now dev`](https://zeit.co/blog/now-dev), but as of this writing, the rust support still needs some work. In the meantime, you could build a simple binary that uses the library, or you can deploy it to your own [`zeit`](https://zeit.co) account. If you need help, please submit an issue and I'll be happy to try and help you out.

## [License](LICENSE.md)
