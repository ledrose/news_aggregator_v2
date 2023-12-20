import { useEffect, useState } from "react"

export default function usePersistentState(key,defaultValue) {
    const [state,setState] = useState(() => localStorage.getItem(key) || defaultValue);
    useEffect(()=>{
        localStorage.setItem(key,state);
    },[key,state]);
    return [state,setState];
}