import './class_diagram.css'
import { Analytics } from "@vercel/analytics/react"

export default function MyApp({ Component, pageProps }) {
    return <div><Component {...pageProps} /><Analytics /></div>;
}
