import { createSlice } from "@reduxjs/toolkit"

const initialState = {
    email: null,
    role: null
}

export const userState = createSlice({
    name: "user",
    initialState,
    reducers: {
        setUser: (state,action) => {
            state.email = action.payload.email;
            state.role = action.payload.role;
        },
        reset: (state) => {
            state.email = null;
            state.role = null;
        }
    }
})

export const {setUser,reset} = userState.actions;
export default userState.reducer;