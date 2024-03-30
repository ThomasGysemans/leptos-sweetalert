# SweetAlert for Leptos

[SweetAlert](https://sweetalert2.github.io/) is a JavaScript library for creating nice popups on the web.

My repo is a crate that allows you to use something similar in the [Leptos](https://leptos.dev) web framework.

> **NOTE**: This is not official. The SweetAlert team did not create this repo. I just wanted to be able to use SweetAlert in Leptos therefore I tried to re-create it myself. IT IS NOT EXACTLY THE SAME THING. I did not simply copy the CSS, I wrote it myself since my goal is not to have a perfect replica of SweetAlert, but to simplify the work for people that would like to have something similar. It is very easy to modify yourself, and if you want to customize it then feel free to simply copy [lib.rs](./src/lib.rs) and the CSS files into your project.

## How to use

This is not ready for production yet. The repo currently holds a [main.rs](./src/main.rs) and its only purpose is to help me test the CSS. When I'm done I will create a proper example and remove the unnecessary dependencies from [Cargo.toml](./Cargo.toml).
