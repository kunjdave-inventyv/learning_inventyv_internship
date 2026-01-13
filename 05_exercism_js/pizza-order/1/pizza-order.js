/// <reference path="./global.d.ts" />
//
// @ts-check

/**
 * Determine the price of the pizza given the pizza and optional extras
 *
 * @param {Pizza} pizza name of the pizza to be made
 * @param {Extra[]} extras list of extras
 *
 * @returns {number} the price of the pizza
 */
export const price = { 
  Margherita: 7,
  Caprese: 9,
  Formaggio: 10,
  ExtraSauce: 1,
  ExtraToppings: 2,
}

export function pizzaPrice(pizza, ...extras) {
  if (pizza === undefined) {
    return 0;
  }
  if (extras.length === 0) {
    return price[pizza];
  }
  return price[pizza] + pizzaPrice(extras[0], ...extras.slice(1));
}

/**
 * Calculate the price of the total order, given individual orders
 * Using imperative loop to avoid stack overflow with large orders
 *
 * @param {PizzaOrder[]} pizzaOrders a list of pizza orders
 * @returns {number} the price of the total order
 */
export function orderPrice(pizzaOrders) {
  let total = 0;
  
  for (const order of pizzaOrders) {
    total += pizzaPrice(order.pizza, ...order.extras);
  }
  
  return total;
}