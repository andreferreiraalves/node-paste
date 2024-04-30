var href: string;

export default function getApiUrl() {

    if (!href)
        href = process.env.NEXT_PUBLIC_API_URL ?? (window.location.origin + '/api')

    return href;
}