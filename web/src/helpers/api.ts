export default function getApiUrl() {
    return process.env.NEXT_PUBLIC_API_URL ?? "";
}