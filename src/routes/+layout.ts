export const ssr = false;
export const prerender = true;
export const csr = true;

export const load = async ({ url: {pathname} }: any) => {
    return { pathname }
};