import detailedDiff from '../src/detailed'
import pureDetailedDiff from '../src_pure_js/detailed'
import addedDiff from '../src/added'
import pureAddedDiff from '../src_pure_js/added'
import { benchmarkSuite } from 'jest-bench'

function randomString (length = null) {
  if (length === null) {
    length = randomNumberBetween(10)
  }
  let result = ''
  let characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'
  let charactersLength = characters.length
  for (let i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() *
      charactersLength))
  }
  return result
}

function randomNumberBetween (to, from = 1) {
  return Math.floor(Math.random() * to) + from
}

function randomValue () {
  const possibilities = [true, false, 'number', 'string']
  const index = randomNumberBetween(possibilities.length - 1, 0)
  let value = possibilities[index]

  if(value === 'number') {
    value = randomNumberBetween(1000000)
  }

  if(value === 'string') {
    value = randomString()
  }

  return value
}

const depthEntryCounts = [100, 20, 10, 5]
const maxDepth = depthEntryCounts.length

function generateEntry(depth) {

  if(depth > maxDepth) {
    return randomValue()
  }

  const entry = {}

  const depthCount = depthEntryCounts[depth]
  for(let i = 0;i<=depthCount;i++) {
    entry[randomString()] = generateEntry(depth + 1)
  }

  return entry
}

const firstObject = generateEntry(0)
const secondObject = generateEntry(0)


benchmarkSuite('.bigDiff', {

  ['detailedDiff']: () => {
    detailedDiff(firstObject, secondObject)
  },

  ['pureDetailedDiff']: () => {
    pureDetailedDiff(firstObject, secondObject)
  },

  ['addedDiff']: () => {
    addedDiff(firstObject, secondObject)
  },

  ['pureAddedDiff']: () => {
    pureAddedDiff(firstObject, secondObject)
  },

})
