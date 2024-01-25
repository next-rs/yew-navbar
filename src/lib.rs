use yew::prelude::*;

const NAVBAR_CLASS: &str = "fixed top-0 left-0 w-full bg-black text-white font-roboto z-20";
const LOGO_CLASS: &str = "flex items-center";
const LOGO_IMG_CLASS: &str = "w-32 md:w-40";
const MENU_TOGGLE_CLASS: &str = "btn-menu ml-4 md:hidden cursor-pointer";
const LINE_CLASS: &str = "line h-1 mb-1 bg-white transition-transform transform origin-center";
const FLEX_CONTAINER_CLASS: &str = "flex justify-between items-center";
const HIDDEN_MD_CLASS: &str = "hidden md:flex nav-wrap";
const NAV_CLASS: &str = "flex flex-grow justify-end items-center space-x-4 md:space-x-8";
const MENU_ITEM_CLASS: &str = "nav-link text-white hover:text-gray-300 transition-colors";
const BUTTON_LINK_CLASS: &str =
    "rounded-full py-2 px-6 bg-blue-500 text-white text-lg transition-colors hover:bg-blue-600";
const DROPDOWN_CLASS: &str =
    "absolute top-full left-0 mt-2 bg-black text-white p-2 rounded shadow-lg block md:hidden";
const DROPDOWN_ITEM_CLASS: &str = "border-b border-blue-500";
const SEARCH_INPUT_CLASS: &str = "hidden md:block rounded-full py-2 px-4 bg-gray-800 text-white text-lg placeholder-gray-500 focus:outline-none";

#[derive(Clone, Properties, PartialEq)]
pub struct NavbarProps {
    // Main props
    #[prop_or_default]
    pub menus: Vec<Menu>,
    #[prop_or_default]
    pub button_href: &'static str,
    #[prop_or_default]
    pub button_text: &'static str,

    // Styling props
    #[prop_or(NAVBAR_CLASS)]
    pub navbar_class: &'static str,
    #[prop_or(LOGO_CLASS)]
    pub logo_class: &'static str,
    #[prop_or(MENU_TOGGLE_CLASS)]
    pub menu_toggle_class: &'static str,
    #[prop_or(LINE_CLASS)]
    pub line_class: &'static str,
    #[prop_or(FLEX_CONTAINER_CLASS)]
    pub flex_container_class: &'static str,
    #[prop_or(HIDDEN_MD_CLASS)]
    pub hidden_md_class: &'static str,
    #[prop_or(NAV_CLASS)]
    pub nav_class: &'static str,
    #[prop_or(MENU_ITEM_CLASS)]
    pub menu_item_class: &'static str,
    #[prop_or_default]
    pub button_class: &'static str,
    #[prop_or(BUTTON_LINK_CLASS)]
    pub button_link_class: &'static str,
    #[prop_or(DROPDOWN_ITEM_CLASS)]
    pub dropdown_item_class: &'static str,
    #[prop_or(DROPDOWN_CLASS)]
    pub dropdown_class: &'static str,
    #[prop_or(SEARCH_INPUT_CLASS)]
    pub search_input_class: &'static str,

    // Logo props
    #[prop_or("images/logo.png")]
    pub logo_src: &'static str,
    #[prop_or("logo")]
    pub logo_alt: &'static str,
    #[prop_or(LOGO_IMG_CLASS)]
    pub logo_img_class: &'static str,
    #[prop_or("/")]
    pub logo_link: &'static str,
}

impl Default for NavbarProps {
    fn default() -> Self {
        Self {
            menus: Default::default(),
            button_href: Default::default(),
            button_text: Default::default(),
            navbar_class: NAVBAR_CLASS,
            logo_class: LOGO_CLASS,
            menu_toggle_class: MENU_TOGGLE_CLASS,
            line_class: LINE_CLASS,
            flex_container_class: FLEX_CONTAINER_CLASS,
            hidden_md_class: HIDDEN_MD_CLASS,
            nav_class: NAV_CLASS,
            menu_item_class: MENU_ITEM_CLASS,
            button_class: Default::default(),
            button_link_class: BUTTON_LINK_CLASS,
            dropdown_item_class: DROPDOWN_ITEM_CLASS,
            dropdown_class: DROPDOWN_CLASS,
            search_input_class: SEARCH_INPUT_CLASS,
            logo_src: "images/logo.png",
            logo_alt: "logo",
            logo_img_class: LOGO_CLASS,
            logo_link: "/",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Menu {
    pub id: usize,
    pub link: &'static str,
    pub name: &'static str,
}

#[function_component(Navbar)]
pub fn navbar_component(props: &NavbarProps) -> Html {
    let is_dropdown_visible = use_state(|| false);

    html! {
        <section id="navbar" class={props.navbar_class}>
            <div class="container mx-auto px-4 py-2">
                { render_navbar_content(props, is_dropdown_visible) }
            </div>
        </section>
    }
}

fn render_navbar_content(props: &NavbarProps, is_dropdown_visible: UseStateHandle<bool>) -> Html {
    html! {
        <div class={props.flex_container_class}>
            { render_logo(props) }
            { render_menu(props) }
            { render_menu_toggle(props, is_dropdown_visible.clone()) }
            { render_dropdown_menu(props, is_dropdown_visible) }
        </div>
    }
}

fn render_logo(props: &NavbarProps) -> Html {
    html! {
        <div id="logo" class={props.logo_class}>
            <a href={props.logo_link} class="nav-link">
                <img src={props.logo_src} alt={props.logo_alt} class={props.logo_img_class} />
            </a>
        </div>
    }
}

fn render_menu_toggle(props: &NavbarProps, is_dropdown_visible: UseStateHandle<bool>) -> Html {
    let onclick = {
        let is_dropdown_visible = is_dropdown_visible.clone();

        Callback::from(move |_| {
            is_dropdown_visible.set(!*is_dropdown_visible);
        })
    };

    html! {
        <div class={props.menu_toggle_class} onclick={onclick}>
            <div class={format!("{} w-6", props.line_class)} />
            <div class={format!("{} w-8", props.line_class)} />
            <div class={format!("{} w-6", props.line_class)} />
        </div>
    }
}

fn render_menu(props: &NavbarProps) -> Html {
    html! {
        <div class={props.nav_class}>
            <div class={props.hidden_md_class}>
                <nav id="mainnav" class="mainnav" data-menu-style="horizontal">
                    <ul class="flex space-x-4 md:space-x-8">
                        { for props.menus.iter().map(|menu| render_menu_item(menu, props)) }
                    </ul>
                </nav>
            </div>
            <input type="text" placeholder="Search..." class={props.search_input_class} />
            { render_button(props) }
        </div>
    }
}

fn render_menu_item(menu: &Menu, props: &NavbarProps) -> Html {
    html! {
        <li key={menu.id}>
            <a class={props.menu_item_class} href={menu.link.to_string()}>{ menu.name }</a>
        </li>
    }
}

fn render_button(props: &NavbarProps) -> Html {
    html! {
        <div class={props.button_class}>
            <a href={props.button_href} class={props.button_link_class} target="_blank">
                <b>{ props.button_text }</b>
            </a>
        </div>
    }
}

fn render_dropdown_menu(props: &NavbarProps, is_dropdown_visible: UseStateHandle<bool>) -> Html {
    if *is_dropdown_visible {
        html! {
            <div class={props.dropdown_class}>
                <ul>
                    { for props.menus.iter().map(|menu| render_dropdown_item(props, menu)) }
                </ul>
            </div>
        }
    } else {
        html! {}
    }
}

fn render_dropdown_item(props: &NavbarProps, menu: &Menu) -> Html {
    html! {
        <li key={menu.id} class={props.dropdown_item_class}>
            <a href={menu.link.to_string()}>{ menu.name }</a>
        </li>
    }
}
