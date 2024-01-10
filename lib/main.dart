import 'package:flutter/material.dart';
import 'package:flutter_rust_newmm/src/rust/api/simple.dart';
import 'package:flutter_rust_newmm/src/rust/frb_generated.dart';

/// Flutter code sample for [FutureBuilder].

// void main() => runApp(const FutureBuilderExampleApp());
Future<void> main() async {
  await RustLib.init();
  runApp(const FutureBuilderExampleApp());
}

class FutureBuilderExampleApp extends StatelessWidget {
  const FutureBuilderExampleApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      home: FutureBuilderExample(),
    );
  }
}

class FutureBuilderExample extends StatefulWidget {
  const FutureBuilderExample({super.key});

  @override
  State<FutureBuilderExample> createState() => _FutureBuilderExampleState();
}

// จำลองใช้เป็นแบบฟังก์ชั่น ให้เสมือนดึงข้อมูลจาก server
Future<String> fetchData() async {
  final response = await Future<String>.delayed(
    const Duration(seconds: 5),
    () => greet(name: 'POP'),
  );
  return response;
}

class _FutureBuilderExampleState extends State<FutureBuilderExample> {
  // final Future<String> _calculation = (await greet(name:"Hi")) as Future<String>;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Future Flutter + Rust'),
      ),
      body: DefaultTextStyle(
        style: Theme.of(context).textTheme.displayMedium!,
        textAlign: TextAlign.center,
        child: FutureBuilder<String>(
          future: fetchData(), // a previously-obtained Future<String> or null
          builder: (BuildContext context, AsyncSnapshot<String> snapshot) {
            List<Widget> children;
            if (snapshot.hasData) {
              children = <Widget>[
                const Icon(
                  Icons.check_circle_outline,
                  color: Colors.green,
                  size: 60,
                ),
                Padding(
                  padding: const EdgeInsets.only(top: 16),
                  child: Text('Result: ${snapshot.data}'),
                ),
              ];
            } else if (snapshot.hasError) {
              children = <Widget>[
                const Icon(
                  Icons.error_outline,
                  color: Colors.red,
                  size: 60,
                ),
                Padding(
                  padding: const EdgeInsets.only(top: 16),
                  child: Text('Error: ${snapshot.error}'),
                ),
              ];
            } else {
              children = const <Widget>[
                SizedBox(
                  width: 60,
                  height: 60,
                  child: CircularProgressIndicator(),
                ),
                Padding(
                  padding: EdgeInsets.only(top: 16),
                  child: Text('Awaiting result...'),
                ),
              ];
            }
            return Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: children,
              ),
            );
          },
        ),
      ),
    );
  }
}
