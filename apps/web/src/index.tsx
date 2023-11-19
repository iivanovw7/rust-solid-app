import "./index.css";
import { lazyImport } from "@/shared";

const { App } = lazyImport(() => import("./app"));

const MOUNT_NODE = document.getElementById("root") as HTMLElement;

/**
 * Renders main application component.
 * @param AppComponent - application component.
 */
const renderApp = (AppComponent: Component) => {
    render(() => <AppComponent />, MOUNT_NODE);
};

if (import.meta.env.DEV && ! (MOUNT_NODE instanceof HTMLElement)) {
    console.error("Root element not found.");
}

/** Renders application at specified mount point. */
renderApp(App);
