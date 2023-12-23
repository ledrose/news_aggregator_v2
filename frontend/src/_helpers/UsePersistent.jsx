import { useEffect, useState } from "react"

export default function usePersistentState(key,defaultValue) {
    const [state,setState] = useState(() => JSON.parse(localStorage.getItem(key)) || defaultValue);
    useEffect(()=>{
        localStorage.setItem(key,JSON.stringify(state));
    },[key,state]);
    return [state,setState];
}

