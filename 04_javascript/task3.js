// let sum = 0;
// function first() {
//   const arr1 = Symbol("arr1");

//   const obj = {
//     [arr1]: [6, 4, 6, 4],
//   };

//   const firstElementOfFirstArray = obj[arr1].shift();

//   return second(firstElementOfFirstArray, obj[arr1]);
// }

// function second(firstElementOfFirstArray, arr1) {
//   const arr2 = Symbol("arr2");

//   const obj = {
//     [arr2]: [5, 5, 5],
//   };

//   return [firstElementOfFirstArray, ...obj[arr2], ...arr1];
// }

// const arr2 = first();

// new Promise( ( resolve , reject) => {
//         let sum = arr2.reduce(( acc , curr ) => {
//             return acc + curr
//         } , 0 )   
//         if (sum >= 35) {
//          resolve(sum)
//         } else {
//          reject("less than 35 ")
//         }
//     }).then( (sum) => {
//         console.log("resolved: sum is -: " , sum)
//     }).catch((error) => { 
//         console.log("rejected:", error);
//     });

arr = [ 2 , 3 , 4 , 5]
newele = 1
console.log([ newele , ...arr ])