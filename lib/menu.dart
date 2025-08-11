import 'package:flutter/material.dart';

class SideMenuPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Row(
        children: [
          Container(
            width: 60,
            color: const Color(0xFFF9F9F9),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                // 顶部图标
                Column(
                  children: [
                    const SizedBox(height: 16),
                    IconButton(
                      icon: const Icon(
                        Icons.desktop_windows,
                        color: Colors.grey,
                      ),
                      onPressed: () {},
                    ),
                    const SizedBox(height: 16),
                    IconButton(
                      icon: const Icon(Icons.storage, color: Colors.grey),
                      onPressed: () {},
                    ),
                    const SizedBox(height: 24),
                    IconButton(
                      icon: const Icon(Icons.history, color: Colors.grey),
                      onPressed: () {},
                    ),
                  ],
                ),

                // 底部图标
                Column(
                  children: [
                    IconButton(
                      icon: const Icon(Icons.settings, color: Colors.black87),
                      onPressed: () {},
                    ),
                    IconButton(
                      icon: const Icon(Icons.apps, color: Colors.black87),
                      onPressed: () {},
                    ),
                    IconButton(
                      icon: Image.asset(
                        'assets/images/github.png',
                        width: 24, // 控制图片大小
                        height: 24,
                        color: Colors.black87,
                      ),
                      onPressed: () {},
                    ),
                    const SizedBox(height: 16),
                  ],
                ),
              ],
            ),
          ),

          // 右侧主内容
          Expanded(
            child: Container(
              color: Colors.white,
              child: const Center(
                child: Text('主内容区域', style: TextStyle(fontSize: 20)),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
