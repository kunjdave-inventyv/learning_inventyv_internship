function first(){
    let arr1 = [ 1 , 5 , 6 , 7 ];
    let firstElemet = arr1[0];
    arr1.shift();
    arr2 = second( firstElemet , arr1 );  
    return arr2;
}

function second( firstElemet , arr1 ){
    arr2 = [ 2 , 3 , 4 ];
    arr2 = [ firstElemet , ...arr2 , ...arr1 ]
    console.log(arr2)
    return arr2;
}

arr = first()
new Promise( ( resolve , reject) => {
        let sum = arr.reduce(( acc , curr ) => {
            return acc + curr
        } , 0 )   
        if (sum >= 35) {
         resolve(sum)
        } else {
         reject("less than 35 ")
        }
    }).then( (sum) => {
        console.log("resolved: sum is -: " , sum)
    }).catch((error) => { 
        console.log("rejected:", error);
    });