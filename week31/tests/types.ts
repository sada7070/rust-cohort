import * as borsh from "borsh";

export class CounterAccount {
    count: number;

    constructor ({count}: {count:number}) {
        this.count = count;
    }
}

export const Schema: borsh.Schema = {
    struct: {
        count: 'u32'
    }
}

export const COUNTER_SIZE = borsh.serialize(Schema, new CounterAccount({count: 0})).length;

console.log(COUNTER_SIZE);