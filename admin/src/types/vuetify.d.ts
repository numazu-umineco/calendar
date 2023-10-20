import Vue from 'vue';

export type VForm = (Vue | Element) & { validate: () => boolean };
