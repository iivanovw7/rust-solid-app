/* eslint-disable @typescript-eslint/no-explicit-any */

export type RunningMode = 'analyze' | 'development' | 'production' | 'test';

export type Pixels = number;

export type Milliseconds = number;

export type Percent = number;

/** Represents type of optional object. */
export type Maybe<T> = T | null | undefined;

/** Represents type of `nullable` object. */
export type Nullable<T> = T | null;

export type ErrorMessage = string;

export type Voidable<T> = T | undefined | void;

export type Recordable<T = any> = Record<string, T>;

export type UnwrapPromise<T extends Promise<any>> = T extends Promise<infer Data> ? Data : never;

/** Represents any function. */
export type AnyFunction = (...args: any[]) => any;

export type AsyncReturnType<T extends (...args: any[]) => Promise<any>> = UnwrapPromise<ReturnType<T>>;

export type Optional<T extends object, K extends keyof T = keyof T> = Omit<T, K> & Partial<Pick<T, K>>;

export type RequiredFields<T, K extends keyof T> = T & Required<Pick<T, K>>;

export type Constructor<T = any> = new (...args: any[]) => T;

export type AugmentedRequired<T extends object, K extends keyof T = keyof T> = Omit<T, K> & Required<Pick<T, K>>;

/** Represents any object object. */
export type AnyObject<T = any> = {
    [field: string]: T;
};

/** Gets property type. */
export type PropType<TObj, TProp extends keyof TObj> = TObj[TProp];

/** Represents type of object with partial and `nullable` fields. */
export type PartialAndNullable<T> = {
    [P in keyof T]?: T[P] | null;
};

export type ObjectOrNull<T = unknown> = Nullable<AnyObject<T>>;

export type OptionalObject<T = unknown> = Maybe<ObjectOrNull<T>>;

/** Object containing promise. */
export type WithPromise<T = unknown> = {
    promise: Promise<T>;
};

export type ValueOf<T> = T[keyof T];

export type ExtractType<T, U extends T> = T extends U ? T : never;

/* eslint-disable @typescript-eslint/no-explicit-any */
