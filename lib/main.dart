import 'package:elasitc_client/menu.dart';
import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'ElasticClient',
      home: const ElasticClientPage(),
    );
  }
}

class ElasticClientPage extends StatefulWidget {
  const ElasticClientPage({super.key});

  @override
  State<ElasticClientPage> createState() => _ElasticClientPageState();
}

class _ElasticClientPageState extends State<ElasticClientPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('ElasticClient')),
      body: SideMenuPage(),
    );
  }
}
