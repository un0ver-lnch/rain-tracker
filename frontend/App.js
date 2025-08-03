import React, { useEffect, useState } from 'react';
import { Text, View, StyleSheet, FlatList } from 'react-native';

export default function App() {
  const [entries, setEntries] = useState([]);

  useEffect(() => {
    fetch('http://localhost:3000/wear-entries')
      .then(res => res.json())
      .then(setEntries)
      .catch(console.error);
  }, []);

  const renderItem = ({ item }) => (
    <View style={styles.item}>
      <Text style={styles.text}>{item.clothing.map(c => c.name).join(', ')}</Text>
      <Text style={styles.subtext}>
        {`${item.weather.temperature_c}°C, ${item.weather.humidity_pct}% - ${item.comfort}`}
      </Text>
    </View>
  );

  return (
    <View style={styles.container}>
      <FlatList
        data={entries}
        keyExtractor={(_, idx) => idx.toString()}
        renderItem={renderItem}
        ListEmptyComponent={<Text style={styles.text}>No entries</Text>}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#121212',
    paddingTop: 50,
    paddingHorizontal: 16,
  },
  text: {
    color: '#fff',
    fontSize: 16,
  },
  subtext: {
    color: '#bbb',
    fontSize: 14,
  },
  item: {
    marginBottom: 16,
  },
});
