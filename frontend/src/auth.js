// import key from './key.json' assert {type: 'json'};
// export function FirebaseAuth() {
//     const firebaseConfig = key

//     firebase.initializeApp(firebaseConfig);
//     const provider = new firebase.auth.GoogleAuthProvider()

//     const loginBtn = document.getElementById('login')

//     firebase.auth().onAuthStateChanged(user => {
//         if (user) {
//             // ユーザーがログインしています。
//             return user;
//         } else {
//             // ユーザーはログインしていません。
//             loginBtn.addEventListener('click', () => {
//                 firebase.auth().signInWithPopup(provider).then(result => {
//                     // GoogleプロパイダのOAuthトークンを取得します。
//                     const token = result.credential.accessToken
//                     // ログインしたユーザーの情報を取得します。
//                     const user = result.user
//                     return {"token": token, "user": user};
//                 }).catch(function(err) {
//                     console.error(err)
//                     // エラー処理
//                 })
//             })
//         }
//     })
// }

// // export function Logout() {
// //     const logoutBtn = document.getElementById('logout')
// //     logoutBtn.addEventListener('click', () => {
// //         firebase.auth().signOut().then(() => {
// //             // 成功
// //         }).catch(err => {
// //             // 失敗
// //         })
// //     })
// // }

