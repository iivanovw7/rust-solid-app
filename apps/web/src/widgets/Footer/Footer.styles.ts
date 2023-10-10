import { cva, cx } from 'cva';

export const footer = cx('sticky', 'top-[100vh]', 'h-8', 'px-2', 'w-full', 'bg-stone-300 dark:bg-stone-800');

export const box = cx('mx-auto', 'max-w-screen-lg w-full h-full', 'flex flex-row justify-between items-center');

export const icon = cva(['block', 'p-0', 'w-5 h-5'], {
    variants: {
        color: {
            error: ['text-red-500'],
            normal: ['text-gray-700 dark:text-gray-300'],
            success: ['text-green-500'],
        },
    },
});

export const link = cx('hover:text-white', 'opacity-80', 'transition-colors', 'p-0');

export const section = cx('flex flex-row items-center gap-2');
