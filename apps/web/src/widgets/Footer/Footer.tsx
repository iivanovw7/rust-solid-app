import { CheckCircle, GithubIcon, XCircle } from 'lucide-solid';

import { healthApi } from '@/shared';

import { box, footer, icon, link, section } from './Footer.styles';

export const Footer = () => {
    const [health] = createResource(healthApi.getHealth);

    return (
        <footer class={footer}>
            <div class={box}>
                <div class={section}>
                    <Switch fallback="...">
                        <Match when={!! health.error}>
                            <XCircle class={icon({ color: 'error' })} />
                        </Match>
                        <Match when={!! health()}>
                            <CheckCircle class={icon({ color: 'success' })} />
                            <span>ver: </span>
                            <span>{health()?.data.version}</span>
                        </Match>
                    </Switch>
                </div>
                <div>
                    <a class={link} href="https://github.com/iivanovw7/rust-solid-app" rel="noreferrer" target="_blank">
                        <GithubIcon class={icon({ color: 'normal' })} />
                    </a>
                </div>
            </div>
        </footer>
    );
};
