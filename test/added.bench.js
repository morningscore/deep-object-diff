import addedDiff from '../src/added';
import rustAddedDiff from '../rust_src/added';
import { benchmarkSuite } from "jest-bench";

benchmarkSuite('.addedDiff', {
/*
    ['equal']: () => {
      test.each([
        ['int', 1],
        ['string', 'a'],
        ['boolean', true],
        ['null', null],
        ['undefined', undefined],
        ['object', { a: 1 }],
        ['array', [1]],
        ['function', () => ({})],
        ['date', new Date()],
      ])('returns empty object when given values of type %s are equal', (type, value) => {
        expect(addedDiff(value, value)).toEqual({});
      })
    },*/

    ['equal']: () => {

      [
        ['int', 1],
        ['string', 'a'],
        ['boolean', true],
        ['null', null],
        ['undefined', undefined],
        ['object', { a: 1 }],
        ['array', [1]],
        ['function', () => ({})],
        ['date', new Date()],
      ].forEach(([type, value]) => {
        expect(addedDiff(type, value)).toEqual({});
      })
    },

    ['rust_equal']: () => {

      [
        ['int', 1],
        ['string', 'a'],
        ['boolean', true],
        ['null', null],
        ['undefined', undefined],
        ['object', { a: 1 }],
        ['array', [1]],
        ['function', () => ({})],
        ['date', new Date()],
      ].forEach(([type, value]) => {
        expect(rustAddedDiff(type, value)).toEqual({});
      })
    }

});
