// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Determines how long it takes to prepare a certain juice.
 *
 * @param {string} name
 * @returns {number} time in minutes
 */
export function timeToMixJuice(name) {
  let time = 0
  if( name === "Tropical Island" ){
    time = 3
  }else if( name === "Green Garden" || name === "Energizer"){
    time = 1.5
  }else if( name === "Pure Strawberry Joy" ){
    time = 0.5
  }else if( name === "All or Nothing" ){
    time = 5
  }else{
    time = 2.5
  }
  return time
}

/**
 * Calculates the number of limes that need to be cut
 * to reach a certain supply.
 *
 * @param {number} wedgesNeeded
 * @param {string[]} limes
 * @returns {number} number of limes cut
 */
export function limesToCut(wedgesNeeded, limes) {
  if( !wedgesNeeded ){
    return 0
  }
  let sum = 0 
  let cnt = 0
  for (let lime of limes) {
    cnt++
    if( lime === "small" ){
      sum += 6
    }
    if( lime === "medium" ){
      sum += 8
    }
    if( lime === "large" ){
      sum += 10
    }
    if( sum >= wedgesNeeded ){
      break
    }
  }
  return cnt
}

/**
 * Determines which juices still need to be prepared after the end of the shift.
 *
 * @param {number} timeLeft
 * @param {string[]} orders
 * @returns {string[]} remaining orders after the time is up
 */
export function remainingOrders(timeLeft, orders) {
  
  for( let i = 0 ; i < orders.length ; i++ ){
    if( timeLeft <= 0 ){
      return orders.slice(i) 
    } 
    timeLeft -= timeToMixJuice(orders[i])
  }
  return []
}
