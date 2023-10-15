import type { AnyObject } from "@packages/types";

/**
 * Named lazy imports, wrapper around solid js `lazy`.
 * @param loader - component loader.
 * @returns new Proxy object containing solid lazy loader method.
 */
export const lazyImport = <T extends AnyObject, U extends keyof T>(loader: (componentName?: string) => Promise<T>) => {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-return
    return new Proxy({} as unknown as T, {
        get: (_target, componentName: string | symbol) => {
            if (typeof componentName === "string") {
                return lazy<Component<unknown>>(() =>
                    loader(componentName).then((ctx) => ({
                        "default": ctx[componentName as U] as Component<unknown>
                    }))
                );
            }
        }
    });
};
