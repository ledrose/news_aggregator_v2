import { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { setError } from "../_store/errorSlice";


function fetchState(isLoading,data,err) {
    return {
        isLoading:isLoading,
        data:data,
        err:err
    }
}


//isLoading,data,error
export default function useCustomFetch(promise,onData=(json)=>{},onErr=(err)=>{}) {
    const [respState,setRespState] = useState(fetchState(false,null,null));
    const dispatch = useDispatch();
    const errAction = (err) => {
        setRespState(fetchState(false,null,err));
        onErr(err);
        dispatch(setError(err.toString()));
    }
    const sendRequest = (...args) => {
        setRespState(fetchState(true,null,null));
        promise(...args).then((response) => {
            if (response.ok) {
                const resp = response.text().then((text)=> {
                    const data = text && JSON.parse(text);
                    if (data === "") {
                        const err = (data && data.message) || response.status_text;
                        errAction(err);
                    } else {
                        setRespState(fetchState(false,data,null));
                        console.log(data);
                        onData(data);
                    }
                });
                console.log("Ok: "+resp);
            }
            else {
                response.text().then((err)=> {
                    errAction(err)
                })
            }
        },(err) => {
            errAction(err);
        });
    };
    return [respState.isLoading,respState.data,respState.err,sendRequest];
}