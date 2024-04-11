import { createSlice } from "@reduxjs/toolkit";

const initialState = {
    error: null,
    type: "error"
}

export const errorState = createSlice({
    name: 'error',
    initialState,
    reducers: {
        setError: (state,action) => {
            state.type = "error"
            state.error = action.payload;
        },
        setInfo: (state,action) => {
            state.type = "info";
            state.error = action.payload;
        },
        reset: (state) => {
            state.error = null;
        }
    }
})

export const {setError, setInfo,reset} = errorState.actions;
export default errorState.reducer;