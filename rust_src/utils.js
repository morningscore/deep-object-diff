import utils from '../crates/utils'

export const isDate = d => d instanceof Date;
export const isEmpty = o => utils.isEmpty(o);
export const isObject = o => o != null && typeof o === 'object';
export const hasOwnProperty = (o, ...args) => Object.prototype.hasOwnProperty.call(o, ...args)
export const isEmptyObject = (o) => isObject(o) && isEmpty(o);
