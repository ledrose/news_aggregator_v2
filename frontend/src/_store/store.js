import {configureStore} from "@reduxjs/toolkit";
import errorReducer from "./errorSlice";
import userReducer from "./userSlice";

const persistedState = localStorage.getItem('reduxStorage')?JSON.parse(localStorage.getItem('reduxStorage')):{};

const store =  configureStore({
    reducer: {
        error: errorReducer,
        user: userReducer,
    },
    persistedState
})

store.subscribe(() => {
    localStorage.setItem("reduxStorage",JSON.stringify(store.getState()));
})

export default store;