import { CheckCircle, GithubIcon, XCircle } from 'lucide-solid';

import { footer, icon, link, section } from './Footer.styles';

const fetchHealth = async () => {
    return (await fetch('/api/health')).json();
};

export const Footer = () => {
    const [health] = createResource(fetchHealth);

    onMount(fetchHealth);

    return (
        <footer class={footer}>
            <div class={section}>
                <span>Api status:</span>
                <Switch fallback="...">
                    <Match when={!! health.error}>
                        <XCircle class={icon({ color: 'error' })} />
                    </Match>
                    <Match when={!! health()}>
                        <CheckCircle class={icon({ color: 'success' })} />
                        <span>version: </span>
                        <span>{JSON.stringify(health().version, null, 1).replaceAll('"', '')}</span>
                    </Match>
                </Switch>
            </div>
            <div>
                <a class={link} href="https://github.com/iivanovw7/rust-solid-app" rel="noreferrer" target="_blank">
                    <GithubIcon class={icon({ color: 'normal' })} />
                </a>
            </div>
        </footer>
    );
};
