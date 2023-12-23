import {configureStore} from "@reduxjs/toolkit";
import errorReducer from "./errorSlice";
import userReducer from "./userSlice";
import storage from "redux-persist/lib/storage"
import {persistStore, persistReducer} from "redux-persist";
import { thunk } from "redux-thunk";

const persistConfig = {
    key: "root",
    storage
}

const persistedReducer = persistReducer(persistConfig,userReducer)

const store =  configureStore({
    reducer: {
        error: errorReducer,
        user: persistedReducer,
    },
    middleware: () => [thunk]
})

export default store;
export const persistor = persistStore(store);