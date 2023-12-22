import { createSlice } from "@reduxjs/toolkit";

const initialState = {
    error: null,
}

export const errorState = createSlice({
    name: 'error',
    initialState,
    reducers: {
        setError: (state,action) => {
            state.error = action.payload;
        },
        reset: (state) => {
            state.error = 0;
        }
    }
})

export const {setError,reset} = errorState.actions;
export default errorState.reducer;