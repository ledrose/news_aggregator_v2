import { createSlice } from "@reduxjs/toolkit"

const initialState = {
    email: null,
    role: null,
    token: null,
    allowed_sources: [],
    current_channel: null,
    channels: []
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
        setAllowedSources: (state,action) => {
            state.allowed_sources = action.payload.sources;
            state.current_channel = action.payload.name;
        },
        resetAllowedSources: (state) => {
            state.allowed_sources = [];
            state.current_channel = null;
        },
        reset: (state) => {
            state.email = null;
            state.role = null;
            state.token = null;
            state.channels = [];
            state.allowed_sources = [];
        },
        setChannel: (state, action) => {
            state.channels = action.payload;
        }
    }
})

export const selectToken = (state) => {
    return state.user.token;
}

export const {setUser,reset, setChannel,setAllowedSources,resetAllowedSources} = userState.actions;
export default userState.reducer;