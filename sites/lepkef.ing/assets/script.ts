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
        if (!data.css) {
            resolve();
        } else {
            const head = document.querySelector('head') as HTMLHeadElement | null;
            if (!head) {
                resolve();
                return;
            }
            const style = document.createElement('link');
            style.rel = 'stylesheet';
            style.href = '/assets/' + data.css;
            head.appendChild(style);
            pageSpecificStyleTags.push(style);

            style.onload = () => {
                resolve();
            };
        }
    });
}

function replaceContent(data: PageData): void {
    const content = document.querySelector<HTMLElement>('.content');
    if (!content) {
        return;
    }
    handleStyleTags(data)
        .then(() => {
            content.innerHTML = data.content;
            registerLinkHandlers();
        });
}

function handleLinkClick(event: Event): void {
    const link = event.currentTarget as HTMLAnchorElement | null;
    if (!link) return;
    const href = link.getAttribute('href') || '';

    // If the link is external, don't do anything.
    if (href.startsWith('http')) {
        return;
    }

    event.preventDefault();
    const content = document.querySelector<HTMLElement>('.content');

    const json = (href === '/' || href === '') ? '/index.json' : href + '.json';
    
    // Check cache first
    if (pageCache.has(json)) {
        const data = pageCache.get(json)!;
        window.history.pushState({}, '', link.href);
        replaceContent(data);
        return;
    }

    document.body.classList.remove('collapsed');
    if (content) {
        content.innerHTML = 'Loading...';
    }
    fetch(json)
        .then((response: Response) => response.json())
        .then((data: PageData) => {
            // Cache the response
            pageCache.set(json, data);
            
            window.history.pushState({}, '', link.href);
            replaceContent(data);
        });
}

function registerLinkHandlers(): void {
    const links = document.querySelectorAll<HTMLAnchorElement>('a');
    links.forEach(link => {
        link.removeEventListener('click', handleLinkClick as EventListener);
        link.addEventListener('click', handleLinkClick as EventListener);
    });
}

// Initialize navigation functionality
document.getElementById('button-collapse')?.addEventListener('click', (event) => {
    document.body.classList.toggle('collapsed');
});

document.getElementById('button-close')?.addEventListener('click', () => {
    document.querySelectorAll('.window').forEach((window) => window.remove());
});

document.getElementById('button-back')?.addEventListener('click', () => {
    window.history.back();
});

document.getElementById('button-forward')?.addEventListener('click', () => {
    window.history.forward();
});

document.getElementById('button-reload')?.addEventListener('click', () => {
    window.location.reload();
});

window.addEventListener('popstate', () => {
    const pathname = location.pathname;
    const json = (pathname === '/' || pathname === '') ? '/index.json' : pathname + '.json';
    const content = document.querySelector<HTMLElement>('.content');
    
    // Check cache first
    if (pageCache.has(json)) {
        const data = pageCache.get(json)!;
        replaceContent(data);
        return;
    }
    
    document.body.classList.remove('collapsed');
    if (content) {
        content.innerHTML = 'Loading...';
    }
    fetch(location.origin + json)
        .then((response: Response) => response.json())
        .then((data: PageData) => {
            // Cache the response
            pageCache.set(json, data);
            replaceContent(data);
        });
});

registerLinkHandlers();