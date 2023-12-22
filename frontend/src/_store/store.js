import {configureStore} from "@reduxjs/toolkit";
import errorReducer from "./errorSlice";
import userReducer from "./userSlice";

export const store = configureStore({
    reducer: {
        error: errorReducer,
        user: userReducer,
    },
})
