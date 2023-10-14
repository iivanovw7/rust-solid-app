import { page } from "./Page.styles";

export type PageProps = ParentProps;

export const Page = (props: ParentProps) => {
    return <div class={page}>{props.children}</div>;
};
