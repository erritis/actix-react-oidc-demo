import React, { useState } from 'react';

const Secured = () => {

  const [userinfo, setUserinfo] = useState<[string, string][]>([]);

  React.useEffect(() => {
      (async () => {
      try {
          const response = await fetch(`${process.env.REACT_APP_BACKEND_URL}/userinfo`);
          let result: any = await response.json();
          
          setUserinfo(Object.entries(result));
      } catch (e) {
          console.error(e);
      }
      })();
  }, []);




 return (
   <div>

    <h1 id="tableLabel">Weather forecast</h1>

    <p>This component demonstrates fetching data from the server.</p>

     { userinfo.length ? (
      <table className='table table-striped' aria-labelledby="tableLabel">
        <thead>
          <tr>
            <th>Key:</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {userinfo.map(item => (
            <tr>
              <td>{ item[0] }</td>
              <td>{ item[1] }</td>
            </tr>
          ))}
        </tbody>
      </table>
    ): 
    (<p><em>Loading...</em></p>) 
    }
   </div>
 );
};

export default Secured;