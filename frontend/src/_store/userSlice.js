import { createSlice } from "@reduxjs/toolkit"

const initialState = {
    email: null,
    role: null,
    token: null
}

export const userState = createSlice({
    name: "user",
    initialState,
    reducers: {
        setUser: (state,action) => {
            state.email = action.payload.email;
            state.role = action.payload.role;
            state.token = action.payload.token;

        },
        reset: (state) => {
            state.email = null;
            state.role = null;
            state.token = null;
        }
    }
})

export const selectToken = (state) => {
    return state.user.token;
}

export const {setUser,reset} = userState.actions;
export default userState.reducer;