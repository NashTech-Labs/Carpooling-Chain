// Import
const express = require('express');
const { ApiPromise, WsProvider } = require('@polkadot/api');

var crypto = require('crypto');

const app = express();

app.get('/index', (req, res) =>{
    async function digestMessage(message) {
        try{
            
            const hash = await crypto.createHash('sha256',message).digest('hex');
            return hash;
        }

        catch(error){
            console.log(error);
        }
        
      }
      
    async function main(){
    
    
        // Construct
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    
        const api = await ApiPromise.create({ provider: wsProvider,
            types: {
                DriverOf: {
                    id: 'u32',
                    car_no: 'Hash',
                    location: ('u32', 'u32'),
                    price: 'u32',
                    destination: ('u32', 'u32')
                },
                CustomerOf: {
                    id: 'u32',
                    name: 'Hash',
                    location: ('u32', 'u32')
                },
              }
        });
        
        try{
            let namePromise = digestMessage("Aman Verma");
            namePromise.then((cust)=>{
                api.tx.carpooling.addNewCustomer(15,{id: 15, name: cust, location: (20,40)});
                console.log(`The customer was successfully added`);
            });
            
        }
        catch(error){
            console.log(error);
        }  
    }   
    main().then(() => console.log('completed'));
    res.send("Done");
});

app.listen(6069);