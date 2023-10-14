import type { AnyObject } from "@internal/types";

/**
 * Named lazy imports, wrapper around solid js `lazy`.
 * @param loader - component loader.
 * @returns new Proxy object containing solid lazy loader method.
 */
export const lazyImport = <T extends AnyObject, U extends keyof T>(loader: (componentName?: string) => Promise<T>) => {
    return new Proxy({} as unknown as T, {
    // eslint-disable-next-line consistent-return
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
