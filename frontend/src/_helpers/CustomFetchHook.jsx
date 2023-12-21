import { useEffect, useState } from "react";


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
    const sendRequest = (...args) => {
        setRespState(fetchState(true,null,null));
        promise(...args).then((response) => {
            console.log(response);
            if (response.ok) {
                const resp = response.text().then((text)=> {
                    const data = text && JSON.parse(text);
                    if (data === "") {
                        const err = (data && data.message) || response.status_text;
                        setRespState(fetchState(false,null,err));
                        onErr(err);
                    } else {
                        setRespState(fetchState(false,data,null));
                        onData(data);
                    }
                });
                console.log("Ok: "+resp);
            }
            else {
                const err = response.status_text;
                setRespState(fetchState(false,null,err));
                onErr(err);
            }
        },(err) => {
            setRespState(fetchState(false,null,err));
            onErr(err);
        });
    };
    return [respState.isLoading,respState.data,respState.err,sendRequest];
}