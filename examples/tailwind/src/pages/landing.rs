use yew::prelude::*;
use yew_navbar::{Menu, Navbar, NavbarProps};

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let navbar_props = NavbarProps {
        navbar_class: "top-0 left-0 w-full bg-gradient-to-r from-blue-500 to-purple-500 text-white font-roboto z-20",
        logo_class: "flex items-center transform hover:scale-110 transition-transform",
        menu_toggle_class: "btn-menu ml-4 md:hidden cursor-pointer rotate-45 transition-transform",
        line_class: "line h-1 mb-1 bg-white transition-transform transform origin-center",
        flex_container_class: "flex justify-between items-center p-4",
        hidden_md_class: "hidden md:flex nav-wrap space-x-4",
        nav_class: "flex flex-grow justify-end items-center space-x-4 md:space-x-8",
        menu_item_class: "nav-link text-white hover:text-gray-300 transition-colors",
        button_class: "bg-yellow-500 hover:bg-yellow-600 rounded-full py-2 px-6 text-white text-lg transition-colors",
        button_text: "Hello",
        logo_img_class: "w-32 md:w-40",
        dropdown_item_class: "border-b border-yellow-500",
        dropdown_class: "absolute top-full left-0 mt-2 bg-black text-white p-2 rounded shadow-lg border border-blue-500 block md:hidden",
        search_input_class: "hidden md:block rounded-full py-2 px-4 bg-gray-800 text-white text-lg placeholder-gray-500 focus:outline-none",
        ..NavbarProps::default()
    };
    let navbar_props1 = NavbarProps {
            navbar_class: "top-28 left-0 w-full bg-blue-500 text-white font-roboto z-20",
            menu_toggle_class: "btn-menu ml-4 md:hidden cursor-pointer",
            button_text: "Hello",
            logo_img_class: "w-32 md:w-40",
            nav_class:
                "flex flex-col md:flex-row justify-end items-center space-y-4 md:space-y-0 md:space-x-8",
            ..NavbarProps::default()
        };

    let navbar_props2 = NavbarProps {
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
            Menu {
                id: 4,
                link: "#contact",
                name: "Contact",
            },
        ],
        button_text: "Hello",
        logo_img_class: "w-32 md:w-40",
        ..NavbarProps::default()
    };

    let navbar_props3 = NavbarProps {
        navbar_class: "top-60 left-0 w-full bg-gradient-to-r from-pink-500 via-purple-500 to-indigo-500 text-white font-roboto z-20",
        logo_class: "flex items-center transform hover:rotate-180 transition-transform",
        logo_img_class: "w-32 md:w-40 animate-bounce",
        button_text: "Hello",
        ..NavbarProps::default()
    };
    let navbar_props4 = NavbarProps {
        navbar_class: "top-80 left-0 w-full bg-gray-900 text-yellow-300 font-mono z-20",
        menu_item_class: "nav-link text-yellow-300 hover:text-gray-300 transition-colors",
        logo_img_class: "w-32 md:w-40",
        button_text: "Hello",
        button_link_class: "rounded-full py-2 px-6 bg-purple-500 text-white text-lg transition-colors hover:bg-purple-600",
        search_input_class: "hidden md:block rounded-full py-2 px-4 bg-gray-800 text-white text-lg placeholder-gray-500 focus:outline-none",
        ..NavbarProps::default()
    };
    let navbar_props5 = NavbarProps {
        navbar_class: "top-100 left-0 w-full bg-gradient-to-b from-blue-500 to-purple-500 text-white font-roboto z-20 overflow-hidden",
        logo_class: "flex items-center",
        logo_img_class: "w-32 md:w-40 animate-pulse",
        line_class: "line h-1 mb-1 bg-white transition-transform transform origin-center animate-bounce",
        button_text: "Hello",
        button_link_class: "rounded-full py-2 px-6 bg-pink-500 text-white text-lg transition-colors hover:bg-pink-600 animate__animated animate__bounce",
        ..NavbarProps::default()
    };
    let navbar_props6 = NavbarProps {
        navbar_class: "top-100 left-0 w-full",
        logo_img_class: "w-32 md:w-40 transform hover:rotate-45 transition-transform duration-500",
        button_text: "Hello",
        ..NavbarProps::default()
    };

    let navbar_props7 = NavbarProps {
        navbar_class: "top-120 left-0 w-full bg-black text-white font-roboto z-20",
        logo_class: "flex items-center transform translate-y-1/4 filter blur-sm",
        logo_img_class: "w-32 md:w-40",
        button_text: "Hello",
        nav_class: "flex flex-grow justify-end items-center space-x-4 md:space-x-8",
        menu_item_class: "nav-link text-white hover:text-gray-300 transition-colors transform translate-y-1/2 delay-75",
        button_link_class: "rounded-full py-2 px-6 bg-blue-500 text-white text-lg transition-colors hover:bg-blue-600 transform translate-y-1/2 delay-150",
        ..NavbarProps::default()
    };
    let navbar_props8 = NavbarProps {
        navbar_class: "top-140 left-0 w-full bg-black text-white font-roboto z-20",
        logo_img_class: "w-32 md:w-40 transform rotate-45",
        line_class:
            "line h-1 mb-1 bg-white transition-all transform hover:translate-x-2 origin-center",
        button_text: "Hello",
        ..NavbarProps::default()
    };
    let navbar_props9 = NavbarProps {
        navbar_class: "top-160 left-0 w-full bg-gradient-to-b from-blue-500 via-purple-500 to-pink-500 text-white font-roboto z-20",
        logo_img_class: "w-32 md:w-40",
        button_text: "Hello",
        ..NavbarProps::default()
    };

    html! {
        <>
          <Navbar ..navbar_props />
          <Navbar ..navbar_props1 />
          <Navbar ..navbar_props2 />
          <Navbar ..navbar_props3 />
          <Navbar ..navbar_props4 />
          <Navbar ..navbar_props5 />
          <Navbar ..navbar_props6 />
          <Navbar ..navbar_props7 />
          <Navbar ..navbar_props8 />
          <Navbar ..navbar_props9 />
        </>
    }
}
