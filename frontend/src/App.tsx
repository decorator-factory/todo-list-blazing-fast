import { Link, Route, Router } from "wouter"
import {
    useLocationProperty,
    navigate,
    LocationHook,
} from "wouter/use-location"

function InboxPage() {
    return <div>Inbox stuff</div>
}

const hashLocation = () => window.location.hash.replace(/^#/, "") || "/"

const hashNavigate = (to: string) => navigate("#" + to)

const useHashLocation: LocationHook = () => {
    const location = useLocationProperty(hashLocation)
    return [location, hashNavigate]
}

export default function App() {
    return (
        <div>
            <Router hook={useHashLocation}>
                <Link href="/users/1">
                    <a className="link">Profile</a>
                </Link>

                <Route path="/about">About Us</Route>
                <Route path="/users/:name">
                    {(params) => <div>Hello, {params.name}!</div>}
                </Route>
                <Route path="/inbox" component={InboxPage} />
            </Router>
        </div>
    )
}
