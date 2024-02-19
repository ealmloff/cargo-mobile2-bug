# Wry android linking bug

This application was generated with cargo mobile2, and then updated to wry 0.35 using [these instructions](https://github.com/tauri-apps/wry/blob/dev/MOBILE.md)

First set up environment variables for Android Studio and NDK:

```sh
export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
export ANDROID_HOME="$HOME/Library/Android/sdk"
export NDK_HOME="$ANDROID_HOME/ndk/25.2.9519653"
```

Then install cargo mobile 2:

```sh
cargo install --git https://github.com/tauri-apps/cargo-mobile2

cargo android open
cargo android build
# Start an pixel 7 pro API 33 emulator in android studio
cargo android run
```

The app crashes with these logs:

```
No implementation found for void com.example.dioxus_mobile_test.WryActivity.create(com.example.dioxus_mobile_test.WryActivity) (tried Java_com_example_dioxus_1mobile_1test_WryActivity_create and Java_com_example_dioxus_1mobile_1test_WryActivity_create__Lcom_example_dioxus_1mobile_1test_WryActivity_2)
1969-12-31 18:00:00.000     0-0     <no-tag>                                                     I  --------- beginning of crash
2024-02-19 12:00:23.023  5304-5304  AndroidRuntime          pid-5304                             D  Shutting down VM
2024-02-19 12:00:23.025  5304-5304  AndroidRuntime          pid-5304                             E  FATAL EXCEPTION: main
Process: com.example.dioxus_mobile_test, PID: 5304
java.lang.UnsatisfiedLinkError: No implementation found for void com.example.dioxus_mobile_test.WryActivity.create(com.example.dioxus_mobile_test.WryActivity) (tried Java_com_example_dioxus_1mobile_1test_WryActivity_create and Java_com_example_dioxus_1mobile_1test_WryActivity_create__Lcom_example_dioxus_1mobile_1test_WryActivity_2)
at com.example.dioxus_mobile_test.WryActivity.create(Native Method)
at com.example.dioxus_mobile_test.WryActivity.onCreate(WryActivity.kt:59)
at android.app.Activity.performCreate(Activity.java:8305)
at android.app.Activity.performCreate(Activity.java:8284)
at android.app.Instrumentation.callActivityOnCreate(Instrumentation.java:1417)
at android.app.ActivityThread.performLaunchActivity(ActivityThread.java:3626)
at android.app.ActivityThread.handleLaunchActivity(ActivityThread.java:3782)
at android.app.servertransaction.LaunchActivityItem.execute(LaunchActivityItem.java:101)
at android.app.servertransaction.TransactionExecutor.executeCallbacks(TransactionExecutor.java:135)
at android.app.servertransaction.TransactionExecutor.execute(TransactionExecutor.java:95)
at android.app.ActivityThread$H.handleMessage(ActivityThread.java:2307)
at android.os.Handler.dispatchMessage(Handler.java:106)
at android.os.Looper.loopOnce(Looper.java:201)
at android.os.Looper.loop(Looper.java:288)
at android.app.ActivityThread.main(ActivityThread.java:7872)
at java.lang.reflect.Method.invoke(Native Method)
at com.android.internal.os.RuntimeInit$MethodAndArgsCaller.run(RuntimeInit.java:548)
at com.android.internal.os.ZygoteInit.main(ZygoteInit.java:936)
```
