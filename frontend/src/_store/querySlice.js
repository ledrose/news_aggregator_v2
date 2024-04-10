import { createSlice } from "@reduxjs/toolkit"

const initialState = {
    query: "",
    add_source: [],
    remove_source: [],
    add_themes: [],
    remove_themes: [],
}


export const userState = createSlice({
    name: "query",
    initialState,
    reducers: {
        set_query: (state,action) => {
            state.query = action.payload;
        },
        add_source: (state, action) => {
            state.remove_source = state.remove_source.filter((el) => el!=action.payload);
            state.add_source.push(action.payload)
        },
        remove_source: (state, action) => {
            state.add_source = state.add_source.filter((el) => el!=action.payload);
            state.remove_source.push(action.payload)
        },
        reset_source: (state, action) => {
            state.add_source = state.add_source.filter((el) => el!=action.payload);
            state.remove_source = state.remove_source.filter((el) => el!=action.payload);
        },
        add_theme: (state,action) => {
            state.remove_themes = state.remove_themes.filter((el) => el!=action.payload);
            state.add_themes.push(action.payload)
        },
        remove_theme: (state,action) => {
            state.add_themes = state.add_themes.filter((el) => el!=action.payload);
            state.remove_themes.push(action.payload)
        },
        reset_theme: (state,action) => {
            state.add_themes = state.add_themes.filter((el) => el!=action.payload);
            state.remove_themes = state.remove_themes.filter((el) => el!=action.payload);
        },
    }
})

export const {set_query,add_source,remove_source,reset_source,add_theme,remove_theme,reset_theme} = userState.actions;
export default userState.reducer;