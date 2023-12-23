import {configureStore} from "@reduxjs/toolkit";
import errorReducer from "./errorSlice";
import userReducer from "./userSlice";
import storage from "redux-persist/lib/storage"
import {persistStore, persistReducer} from "redux-persist";

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
})

export default store;
export const persistor = persistStore(store);