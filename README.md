# 🍔 Yew Navbar

[![Crates.io](https://img.shields.io/crates/v/yew-navbar)](https://crates.io/crates/yew-navbar)
[![Crates.io Downloads](https://img.shields.io/crates/d/yew-navbar)](https://crates.io/crates/yew-navbar)
![Crates.io License](https://img.shields.io/crates/l/yew-navbar)
![Rust](https://img.shields.io/badge/rust-stable-orange)

---

![Demo](https://github.com/wiseaidev/yew-navbar/assets/62179149/3e322002-844f-4abd-b4da-ba985ab127cb)

---

## 📜 Introduction

Yew Navbar is a highly customizable and responsive navbar component for the Yew framework. It provides a feature-rich navigation bar that can be easily integrated into your Yew applications.

## 🤔 Why is this Component Useful?

This navbar component offers several benefits to enhance your web application's navigation experience:

1. 🌐 Customization: Tailor the appearance and behavior of the navbar to suit your application's design.

1. 🚀 Responsive Design: Ensures optimal viewing and navigation across various devices and screen sizes.

1. 💬 Easy Integration: Seamless integration into Yew projects with minimal setup and configuration.

## ⚙️ Installation

Integrating Yew Navbar into your Yew project is a straightforward process. Follow these steps:

1. Make sure you have Yew set up in your project. If not, refer to the [Yew documentation](https://yew.rs/docs/getting-started/introduction) for installation instructions.

1. Install the library using your preferred package manager:

   ```bash
   $ cargo add yew-navbar
   ```

1. Start using the navbar component to enhance your application's navigation.

## 🛠️ Usage

Incorporating Yew Navbar into your application is easy. Follow these steps:

1. Import the navbar component into your Yew project:

   ```rust
   use yew::prelude::*;
   use yew_navbar::{Navbar, NavbarProps, Menu};

   #[function_component(App)]
   pub fn app() -> Html {
       // tailwind css utility classes
       let navbar_props = NavbarProps {
           navbar_class: "top-48 left-0 w-full bg-blue-500 text-white font-roboto z-20",
           dropdown_class: "absolute top-full left-0 mt-2 bg-black text-white p-4 rounded shadow-lg border border-blue-500 grid grid-cols-4 gap-4 block md:hidden",
           dropdown_item_class: "border-b border-blue-500 col-span-1",
           menus: vec![
               Menu {
                   id: 1,
                   link: "#",
                   name: "Home",
               },
               Menu {
                   id: 2,
                   link: "#about",
                   name: "About",
               },
               Menu {
                   id: 3,
                   link: "#services",
                   name: "Services",
               },
           ],
           button_text: "Hello",
           logo_img_class: "w-32 md:w-40",
           ..NavbarProps::default()
       };

       html! {
           <Navbar ..navbar_props />
       }
   }
   ```

1. Customize the navbar appearance and behavior using provided props.

1. Enjoy an enhanced navigation experience with Yew Navbar.

## 🔧 Props

### Main Props

| Name          | Type             | Description                           | Example                   | Default Value   |
| ------------- | ---------------- | ------------------------------------- | ------------------------- | --------------- |
| `menus`       | `Vec<Menu>`      | List of navigation menu items.         | -                         | `Vec::new()`    |
| `button_href` | `&'static str`   | Href for the button link.             | `"wiseai.dev"`           | `""`            |
| `button_text` | `&'static str`   | Text for the button.                  | `"Click me"`              | `""`            |

### Styling Props

| Name                  | Type             | Description                           | Example                   | Default Value   |
| --------------------- | ---------------- | ------------------------------------- | ------------------------- | --------------- |
| `navbar_class`        | `&'static str`   | CSS class for the navbar section.     | `"navbar"`                | `""`            |
| `logo_class`          | `&'static str`   | CSS class for the logo section.       | `"logo"`                  | `""`            |
| `menu_toggle_class`   | `&'static str`   | CSS class for the menu toggle button. | `"menu-toggle"`           | `""`            |
| `line_class`          | `&'static str`   | CSS class for lines in the button.    | `"line"`                  | `""`            |
| `flex_container_class`| `&'static str`   | CSS class for the flex container.     | `"flex-container"`        | `""`            |
| `hidden_md_class`     | `&'static str`   | CSS class for hiding elements on small screens. | `"hidden-md"`    | `""`            |
| `nav_class`           | `&'static str`   | CSS class for the navigation section. | `"nav"`                   | `""`            |
| `menu_item_class`     | `&'static str`   | CSS class for navigation menu items.  | `"menu-item"`             | `""`            |
| `button_class`        | `&'static str`   | CSS class for the button.             | `"button"`                | `""`            |
| `button_link_class`   | `&'static str`   | CSS class for the button link.        | `"button-link"`           | `""`            |
| `dropdown_item_class` | `&'static str`   | CSS class for dropdown menu items.    | `"dropdown-item"`         | `""`            |
| `dropdown_class`      | `&'static str`   | CSS class for the dropdown menu.      | `"dropdown"`              | `""`            |
| `search_input_class`  | `&'static str`   | CSS class for the search input.       | `"search-input"`          | `""`            |

### Logo Props

| Name           | Type             | Description                           | Example                   | Default Value   |
| -------------- | ---------------- | ------------------------------------- | ------------------------- | --------------- |
| `logo_src`     | `&'static str`   | Source path for the logo image.       | `"images/logo.png"`       | `""`            |
| `logo_alt`     | `&'static str`   | Alt text for the logo image.          | `"Logo Alt Text"`         | `""`            |
| `logo_img_class`| `&'static str`   | CSS class for the logo image.         | `"logo-img"`              | `""`            |
| `logo_link`    | `&'static str`   | Href for the logo link.               | `"/home"`                 | `""`            |

## 📙 Examples

If you're curious about how to use it with different styling or additional features, you can check out the [examples folder](examples/tailwind) for more information.

## 🤝 Contribution

We welcome contributions from the community to enhance this Yew Navbar component. Feel free to open issues, submit pull requests, or provide feedback. Let's collaborate to make navigation in Yew even more stylish and functional!

## 📜 License

Yew Navbar is licensed under the `MIT` License, allowing you to use, modify, and distribute it freely. Refer to the [`LICENSE`](LICENSE) file for more details.
