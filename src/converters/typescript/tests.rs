use super::strip_typescript_types;

#[test]
fn strips_router_like_types() {
    let ts = r#"
export {};

interface PageData {
	content: string;
	css?: string;
}

const pageSpecificStyleTags: HTMLLinkElement[] = [];
const pageCache: Map<string, PageData> = new Map();

function handleStyleTags(data: PageData): Promise<void> {
	pageSpecificStyleTags.forEach((style: HTMLLinkElement) => style.remove());
	return new Promise<void>((resolve) => {
		const head = document.querySelector('head') as HTMLHeadElement | null;
		const style = document.createElement('link');
		pageSpecificStyleTags.push(style as HTMLLinkElement);
		(style as HTMLLinkElement).onload = () => {};
	});
}

function replaceContent(data: PageData): void {
	const content = document.querySelector<HTMLElement>('.content');
}

function handleLinkClick(event: Event): void {
	const link = event.currentTarget as HTMLAnchorElement | null;
	fetch('/x').then((response: Response) => response.json()).then((data: PageData) => {});
}

(function(){
	const links = document.querySelectorAll<HTMLAnchorElement>('a');
	const data = pageCache.get('k')!;
})();
		"#;

    let js = strip_typescript_types(ts);

    assert!(!js.contains("interface PageData"));
    assert!(!js.contains(": HTMLLinkElement[]"));
    assert!(!js.contains(": Map<string, PageData>"));
    assert!(!js.contains(": PageData"));
    assert!(!js.contains("Promise<void>"));
    assert!(!js.contains("as HTMLHeadElement"));
    assert!(!js.contains("as HTMLLinkElement"));
    assert!(!js.contains("as EventListener"));
    assert!(!js.contains("<HTMLElement>"));
    assert!(!js.contains("<HTMLAnchorElement>"));
    assert!(!js.contains(")!"));

    // Spot-check a few expected transformations
    assert!(js.contains("const pageSpecificStyleTags = []"));
    // Avoid brittle formatting checks for browser APIs; converter is covered by copier tests as well
}

#[test]
fn preserves_non_ascii_emoji_in_template() {
    let ts = "console.log(`🎉 CORNER HIT! Frame ${frameCount}`);";
    let js = strip_typescript_types(ts);
    assert!(js.contains("🎉 CORNER HIT!"));
}

#[test]
fn preserves_url_in_string_literal() {
    let ts = r#"
(function(){
	setInterval(function(){
		const u='https://static.llllllllllll.com/andor/assets/clippy/swaying.gif?c=' + Date.now();
		console.log(u);
	},8000);
})();
		"#;
    let js = strip_typescript_types(ts);
    assert!(js.contains("https://static.llllllllllll.com/andor/assets/clippy/swaying.gif?c="));
}

#[test]
fn does_not_strip_type_like_sequences_inside_strings_and_templates() {
    let ts = r#"
(function(){
	const a = "as HTMLLinkElement : number <T> ! interface X { a: string }";
	const b = 'querySelector<HTMLElement> as Type : string !';
	const c = `template keeps as Cast<T> : number and bang!`;
	console.log(a,b,c);
})();
		"#;
    let js = strip_typescript_types(ts);
    assert!(js.contains("as HTMLLinkElement : number <T> ! interface X { a: string }"));
    assert!(js.contains("querySelector<HTMLElement> as Type : string !"));
    assert!(js.contains("template keeps as Cast<T> : number and bang!"));
}
