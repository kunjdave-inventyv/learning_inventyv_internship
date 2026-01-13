/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Implement the functions needed to solve the exercise here.
 * Do not forget to export them so they are available for the
 * tests. Here an example of the syntax as reminder:
 *
 * export function yourFunction(...) {
 *   ...
 * }
 */
export function cookingStatus(timeLeft) {
  if (timeLeft === undefined) {
    return 'You forgot to set the timer.'
  }
  if (timeLeft === 0) {
    return 'Lasagna is done.'
  }
  return 'Not done, please wait.'
}
export function preparationTime(layers, avgTime = 2) {
  return layers.length * avgTime
}
export function quantities(layers) {
  let noodles = 0
  let sauce = 0

  for (const layer of layers) {
    if (layer === 'noodles') noodles += 50
    if (layer === 'sauce') sauce += 0.2
  }

  return { noodles, sauce }
}
export function addSecretIngredient(friendsList, myList) {
  myList.push(friendsList[friendsList.length - 1])
}
export function scaleRecipe(recipe, portions) {
  const scaledRecipe = {}
  const factor = portions / 2

  for (const key in recipe) {
    scaledRecipe[key] = recipe[key] * factor
  }

  return scaledRecipe
}

